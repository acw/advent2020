use advent2020::errors::{GrammarParseError, TopLevelError};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

struct Grammar {
    rules: HashMap<usize, Rule>,
}

impl Grammar {
    fn new() -> Grammar {
        Grammar {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, number: usize, rule: Rule) -> Result<(), GrammarParseError> {
        if self.rules.insert(number, rule).is_some() {
            Err(GrammarParseError::DuplicateRule(number))
        } else {
            Ok(())
        }
    }

    fn parses(&self, s: &str) -> Result<bool, GrammarParseError> {
        self.accepts(0, s)
            .map(|x| x.iter().filter(|x| x.is_empty()).count() > 0)
    }

    fn accepts<'a>(&self, rule: usize, s: &'a str) -> Result<Vec<&'a str>, GrammarParseError> {
        match self.rules.get(&rule) {
            None => Err(GrammarParseError::UnknownRule(rule)),
            Some(v) => self.rule_accepts(v, s),
        }
    }

    fn rule_accepts<'a>(&self, rule: &Rule, s: &'a str) -> Result<Vec<&'a str>, GrammarParseError> {
        match rule {
            Rule::Alternatives(alts) => {
                let mut results = Vec::new();

                for item in alts.iter() {
                    let mut news = self.rule_accepts(item, s)?;
                    results.append(&mut news);
                }

                Ok(results)
            }

            Rule::Sequence(seqs) => {
                let mut results = vec![s];

                for item in seqs.iter() {
                    let mut new_results = vec![];

                    for early_result in results.drain(..) {
                        let mut nexts = self.rule_accepts(item, early_result)?;
                        new_results.append(&mut nexts);
                    }

                    results = new_results;
                }

                Ok(results)
            }

            Rule::Nonterminal(new_rule) => self.accepts(*new_rule, s),

            Rule::Terminal(term) => {
                if let Some(rest) = s.strip_prefix(term) {
                    Ok(vec![rest])
                } else {
                    Ok(vec![])
                }
            }
        }
    }

    fn rewrite(&self) -> Grammar {
        let rule8 = Rule::Alternatives(vec![
            Rule::Nonterminal(42),
            Rule::Sequence(vec![Rule::Nonterminal(42), Rule::Nonterminal(8)]),
        ]);

        let rule11 = Rule::Alternatives(vec![
            Rule::Sequence(vec![Rule::Nonterminal(42), Rule::Nonterminal(31)]),
            Rule::Sequence(vec![
                Rule::Nonterminal(42),
                Rule::Nonterminal(11),
                Rule::Nonterminal(31),
            ]),
        ]);

        let mut rules = self.rules.clone();
        rules.insert(8, rule8);
        rules.insert(11, rule11);
        Grammar { rules }
    }
}

#[derive(Clone)]
enum Rule {
    Alternatives(Vec<Rule>),
    Sequence(Vec<Rule>),
    Nonterminal(usize),
    Terminal(String),
}

impl Rule {
    fn new(s: &str) -> Result<(usize, Rule), GrammarParseError> {
        let mut parts = s.split(": ");
        let rule_num_str = parts
            .next()
            .ok_or_else(|| GrammarParseError::BadRule(s.to_string()))?;
        let rule_num = usize::from_str(rule_num_str)?;
        let definitions = parts
            .next()
            .ok_or_else(|| GrammarParseError::BadRule(s.to_string()))?;
        let mut alternatives = Vec::new();

        for alternate in definitions.split(" | ") {
            let trimmed_alternate = alternate.trim();
            let mut sequence_members = Vec::new();

            for member in trimmed_alternate.split_ascii_whitespace() {
                let item = if member.starts_with('"') && member.ends_with('"') {
                    Rule::Terminal(
                        member
                            .strip_prefix('"')
                            .unwrap()
                            .strip_suffix('"')
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    Rule::Nonterminal(usize::from_str(member)?)
                };
                sequence_members.push(item);
            }

            match sequence_members.len() {
                0 => return Err(GrammarParseError::BadRule(s.to_string())),
                1 => alternatives.push(sequence_members.pop().unwrap()),
                _ => alternatives.push(Rule::Sequence(sequence_members)),
            }
        }

        match alternatives.len() {
            0 => Err(GrammarParseError::BadRule(s.to_string())),
            1 => Ok((rule_num, alternatives.pop().unwrap())),
            _ => Ok((rule_num, Rule::Alternatives(alternatives))),
        }
    }
}

macro_rules! run_parser {
    ($parser: ident, $line: ident, $count: ident) => {
        if $parser.parses($line)? {
            $count += 1;
            "YES"
        } else {
            "NO"
        }
    };
}

#[test]
fn rewrite_test() {
    let line = "aaaaabbaabaaaaababaa";
    let contents = fs::read_to_string("inputs/day19_test2.txt").unwrap();
    let mut grammar = Grammar::new();
    for line in contents.lines() {
        if line == "" {
            break;
        }

        let (num, rule) = Rule::new(line).unwrap();
        grammar.add_rule(num, rule).unwrap();
    }
    let rewritten = grammar.rewrite();
    assert_eq!(Ok(false), grammar.parses(line));
    assert_eq!(Ok(true), rewritten.parses(line));
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut grammar = Grammar::new();
    let mut lines = contents.lines();

    for line in &mut lines {
        if line == "" {
            break;
        }

        let (num, rule) = Rule::new(line)?;
        grammar.add_rule(num, rule)?;
    }

    let mut matched_orig_lines = 0;
    let mut matched_rewritten_lines = 0;
    let rewritten_grammar = grammar.rewrite();

    for line in &mut lines {
        let orig = run_parser!(grammar, line, matched_orig_lines);
        let rewritten = run_parser!(rewritten_grammar, line, matched_rewritten_lines);
        println!("{} ==> {} / {}", line, orig, rewritten);
    }

    println!("{} lines matched originally.", matched_orig_lines);
    println!(
        "{} lines matched after it was rewritten.",
        matched_rewritten_lines
    );

    Ok(())
}

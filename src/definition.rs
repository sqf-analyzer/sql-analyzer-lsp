use sqf::{analyzer::Origin, analyzer::State, span::Span};

fn in_span((start, end): Span, offset: usize) -> bool {
    offset >= start && offset < end
}

pub fn get_definition(state: &State, offset: usize) -> Option<Origin> {
    state
        .origins
        .iter()
        .find_map(move |(k, v)| in_span(*k, offset).then_some(v.clone()))
}

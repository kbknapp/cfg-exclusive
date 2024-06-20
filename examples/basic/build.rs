use cfg_exclusive::cfg_exclusive;

cfg_exclusive! {
    validate_feats,
    ["feat1", "feat2", "feat3"],
    "Only one of the features can be enabled at a time"
}

fn main() {
    validate_feats();
}

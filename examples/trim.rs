use bullet_exchange_interface::schema::{Schema, SchemaFile, trim};
use bullet_exchange_interface::transaction::Transaction;

fn main() {
    let filter_variants = |name: &str, variant: &str| match name {
        "Transaction" => variant == "V0",
        "RuntimeCall" => matches!(variant, "Exchange" | "Bank"),
        "CallMessage" => matches!(variant, "Keeper" | "TransferWithMemo"),
        "KeeperAction" => matches!(
            variant,
            "UpdateOraclePrices" | "UpdateMarkPrices" | "UpdateOraclePricesWithPythProofs"
        ),
        "UniquenessData" => variant == "Generation",
        _ => panic!("'{name}::{variant}' is unknown"),
    };

    let our_schema = Schema::of_single_type::<Transaction>().unwrap();
    let remote: SchemaFile = serde_json::from_str(include_str!("schema.json")).unwrap();

    let left = trim(&our_schema, &filter_variants);
    let right = trim(&remote.schema, &filter_variants);

    let left = serde_json::to_string_pretty(&left).unwrap();
    let right = serde_json::to_string_pretty(&right).unwrap();
    let mut faults = 0usize;
    for (l, r) in left.lines().zip(right.lines()) {
        if l != r {
            faults = faults.saturating_add(1);
            println!("< {r}");
            println!("> {l}");
            if faults >= 10 {
                break;
            }
        } else {
            println!("  {l}");
        }
    }
    assert_eq!(left, right);
}

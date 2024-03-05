use gear_wasm_builder::WasmBuilder;
use io::ContractMetadata;
use gmeta::Metadata;

fn main(){
   WasmBuilder::with_meta(ContractMetadata::repr())
      .exclude_features(vec!["binary-vendor"])
      .build();
}
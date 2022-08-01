use super::*;
use ckb_testtool::builtin::ALWAYS_SUCCESS;
use ckb_testtool::ckb_hash::new_blake2b;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let contract_bin: Bytes = Loader::default().load_binary("ckb-project-type-id");
    let type_id_out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, Bytes::from(vec![42]))
        .unwrap();

    // cell deps
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();
    let type_script_dep = CellDep::new_builder()
        .out_point(type_id_out_point.clone())
        .build();

    // prepare inputs
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // type id
    let mut type_id = [0u8; 32];
    let mut blake2b = new_blake2b();
    blake2b.update(input.as_slice());
    blake2b.update(&[0u8; 8]);
    blake2b.finalize(&mut type_id);

    // c-cell type script
    let type_script = context.build_script(&type_id_out_point, Bytes::from(type_id.to_vec()));
    let type_script = type_script.pack();

    // outputs
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(type_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
    ];
    let outputs_data = vec![Bytes::from(vec![42; 2]), Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

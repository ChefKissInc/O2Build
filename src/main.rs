/*
   ,,,,,,,,:::;;s3X,:;.;33:,,.;:..;;..:i:,;.;;,..;i,,s;.,;;,.;A53333
   ,,,,,,,,,,::::r3r.i;.;r,;:;i;::ii::;i:,,:i:..:i:.i;.:i:.:Xhhh5AA5
   ,,,,,,,,,,,::::;i,:;;;;iirrrrrrrrrrriiiiii;;;i;,.,,;;,.;A22AXXsir
   ,,,,,,,,,,,,,,,:,:,,;;iiiiiiiiiiiiiiiirriirrrri;::;:,,;rrrrrrrrri
   ,,,,,,,,,,,,,,,,,::,,;iiiiiiiiiiiiiiiiiiiiiriii;;;:..sAAAAAAXXXXX
   ,,,,,,,,,,,,,,,,,:;;,,irirrriiiiiiiiiiiiiirrrii;;;,.:533333333333
   ,,,,,,,,,,,,,,,.,:;ii,:rriirriiiiiiirriirrrrii;;;;,.;5225h5553335
   ,,,,,,,,,,,,,,,,:;s3Ms,;riiiiiiiiiir;:::;irrii;;;;,.rAAAA55553333
   ,,,,,,,,,,,,,,,:;AMMMM;,iriiiiiiiiii:;;;:;rrii;;;:,.s2AAAA2555533
   ,,,,,,,,,,,,,,:;XMMM3Ar,:iriiiiiiiri,:;;::rri;;;;:. s5A5553333335
   ,,,,,,,,,,,,.,;rhMMXs3GA.:rriiiiiiii;:::;irii;;;;:..s555335AXXssX
   :::::,,,,,,.,:;AMMsX#939X.;rriiiiiiirrrrrrrr;,:;;:,.r352Xriiiirri
   ;;;;;;;:::,.::;5Mh;3@S2B#:.;rriiiiiiirrirrii:.:;;:,.i2s;;rX255555
   ;;;;;;;;;;:,:;;XMMsihS9H2Ar,:irriiiiirrrrri;,,;;;:,.:i;s255555555
   i;;;i;;;ii,,;;;iAMMArrsX5MMs,,:irrrrrrrrr;:.,;;;;:,..iA555555522X
   iii;iiiiii,:;;;;irA3MMMMMh5r;;,,:;;ii;;;,,;2r;;;;:,.,25555552si:,
   iiiiiiiiii,:;;;;ii;irsXXsiiiiri;::,,:;::sM9@G;;;;:,.:555552X;,,,,
   iiiiiii;ii,::;;;iiiiiiiiiiiiiiirri;H9B3,i&@@Bi;;::,.,2555A;,,,,::
   iiiiii;iii,:::;;;iiiirrriiiiiiiiri;H@@BHS&@@M;;;::,.:535X;,,,::,,
   iiiii;;iii:,::;;;;iirrrriiiiiiiiii;rH@@@@@@Hi;;::,..s35s:,,,,::::
   ;iiiiiiiii;,:::;;;;;;;iiiiirriiiirriiAhM32si;;;::,.:25X:,,,,:,:::
   iii;iiiiiii,,::;:,;3ri3X:ri;iirrrririiii;;;;;;::,..s22r,.,,::,,,,
   iiiiiiiiiii;,:::;,;A;iMsr&3;As;;irriiiiii;;;:::,..:X22i,.,::::,::
   ii;;iiiiiiii:,::;;,;2;;;;s;iHX;2siiiii;;;;;:::,.,i:r22s:,,,,::,::
   iii;;;;;;::::,.,::::r:3h:sr::,s#hi:;;;;;:::,,..:XA;;252s:,,,:::,:
   iii;,...........,,::::;;:5r:33;;:::,:;;::,,..;s222s;s552X;,,,,,,:
   i;;ii;,...,:::;::,,,,:::::::ii::::::::,,...;X222222s;rA22As;:,,,,
   ;;;;;;;;;:,.,:;iiii;:,,,,,::::::::,,,.,,:;iisA22222As;;sA2AAXXsi:
   ;;irrsX25A:.,:;iiiirri;;;::,,,,.......;irriiirsA222A2Xi:iXA2A2hh5
*/

#![allow(non_snake_case)]

use std::{fs, path::Path};

use debug_tree::add_branch;
use inkwell::{
    context::Context,
    passes::{PassManager, PassRegistry},
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetTriple},
    OptimizationLevel,
};
use MolecularRearranger::{ast::SyntaxTree, generator::Compiler, get_config, tokeniser::Tokeniser};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "You must provide a path to an oxygen file");
    println!("Infusing: {}", args[1]);

    let contents = fs::read_to_string(&args[1]).expect("Failed to read file");
    add_branch!("Module {}", args[1]);
    // defer_print!();

    // Tokenise text
    let (tokens, errs) = contents.tokenise();
    println!("{:?}\n", tokens);
    println!("Errors: {:?}\n", errs);

    // Parse AST
    let (program, errs) = SyntaxTree::new(tokens);
    println!("{:#?}\n", program);
    println!("Errors: {:?}\n", errs);

    // LLVM stuffs
    let context = Context::create();
    let module = context.create_module(get_config(&args[1]).unwrap());
    module.set_source_file_name(&args[1]);
    let builder = context.create_builder();

    let pass_registry = PassRegistry::get_global();
    pass_registry.initialize_core();
    pass_registry.initialize_transform_utils();
    pass_registry.initialize_scalar_opts();
    pass_registry.initialize_obj_carc_opts();
    pass_registry.initialize_vectorization();
    pass_registry.initialize_inst_combine();
    pass_registry.initialize_ipo();
    pass_registry.initialize_instrumentation();
    pass_registry.initialize_analysis();
    pass_registry.initialize_ipa();
    pass_registry.initialize_codegen();
    pass_registry.initialize_target();

    let func_pm = PassManager::create(&module);
    func_pm.add_verifier_pass();
    func_pm.add_custom_pass("lint");
    func_pm.add_instruction_combining_pass();
    func_pm.add_reassociate_pass();
    func_pm.add_new_gvn_pass();
    func_pm.add_cfg_simplification_pass();
    func_pm.add_basic_alias_analysis_pass();
    func_pm.add_promote_memory_to_register_pass();
    func_pm.add_instruction_combining_pass();
    func_pm.add_reassociate_pass();
    func_pm.initialize();
    let mpm = PassManager::create(());
    mpm.add_custom_pass("instprof");

    Target::initialize_x86(&InitializationConfig::default());

    let triple = TargetTriple::create("x86_64-apple-darwin");
    module.set_triple(&triple);
    let target = Target::from_triple(&triple).expect("Failed to get target from target triple");
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());

    let compiler = Compiler::new(
        &context,
        &builder,
        &func_pm,
        &module,
        target_machine.get_target_data(),
    );
    let res = compiler.compile_program(&program);
    mpm.run_on(&module);

    if let Ok(ir) = res {
        println!("\nLLVM IR:\n{}", ir);

        target_machine
            .write_to_file(&module, FileType::Assembly, Path::new("Build/out.as"))
            .expect("Failed to generate assembly");

        target_machine
            .write_to_file(&module, FileType::Object, Path::new("Build/out.o"))
            .expect("Failed to generate object file");

        Ok(())
    } else {
        compiler.module.print_to_stderr();
        Err(format!("Failed to compile program: {}", res.unwrap_err()))
    }
}

fn main() {
    // "xml_preloader"ディレクトリ内のMakefileを呼び出して前処理を行う
    let make_result = std::process::Command::new("make")
        .current_dir("xml_preload")
        .status();

    if let Err(err) = make_result {
        eprintln!("Error running make: {}", err);
        std::process::exit(1);
    }

    // Makeコマンドが正常終了した場合、ビルドプロセスを続行
    println!("cargo:rerun-if-changed=xml_preloader");
}

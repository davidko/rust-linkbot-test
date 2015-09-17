
fn main() {
    println!("cargo:rustc-link-lib=static=baromesh");
    println!("cargo:rustc-link-lib=static=baromesh-common");
    println!("cargo:rustc-link-lib=static=commontypes-proto");
    println!("cargo:rustc-link-lib=static=daemon-interface");
    //println!("cargo:rustc-link-lib=static=dongle-interface");
    println!("cargo:rustc-link-lib=static=robot-interface");
    println!("cargo:rustc-link-lib=static=rpc-proto");
    println!("cargo:rustc-link-lib=static=rpc");
    println!("cargo:rustc-link-lib=static=sfp");

    let libpath = std::env::current_dir().unwrap();
    let mut ll_libpath = libpath.clone();
    ll_libpath.push("stage-liblinkbot");
    ll_libpath.push("lib");
    ll_libpath.push("static");
    println!("cargo:rustc-link-search=native={}", &ll_libpath.to_str().unwrap());
    
    let mut boost_libpath = libpath.clone();
    let path = vec!["deps", "boost_1_57_0", "stage", "lib"];
    for p in path {
        boost_libpath.push(p)
    }
    println!("cargo:rustc-link-search=native={}", &boost_libpath.to_str().unwrap());
    let boost_libs = vec![ "libboost_chrono.a",  
                           "libboost_date_time.a",
                           "libboost_filesystem.a",
                           "libboost_log.a",
                           "libboost_log_setup.a",
                           "libboost_program_options.a",
                           "libboost_regex.a",
                           "libboost_system.a",
                           "libboost_thread.a" ];
    for l in boost_libs {
        let libname = l.trim_left_matches("lib").trim_right_matches(".a");
        println!("cargo:rustc-link-lib=static={}", libname);
    }

    println!("cargo:rustc-link-lib=stdc++");
}

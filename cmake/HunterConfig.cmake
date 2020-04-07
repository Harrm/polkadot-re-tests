
hunter_config(
    Boost
    VERSION 1.70.0-p0
    CMAKE_ARGS CMAKE_POSITION_INDEPENDENT_CODE=ON
)

hunter_config(sr25519
    URL https://github.com/Warchant/sr25519-crust/archive/1.0.1.tar.gz
    SHA1 3005d79b23b92ff27848c24a7751543a03a2dd13
#    URL https://github.com/Warchant/sr25519-crust/archive/2947abb8367d57cd712e8bc80687d224ccd86ccf.zip
#    SHA1 2b0f06efba6846fd66f8de397179b1b955af8da6
    )

hunter_config(
    GTest
    VERSION 1.8.0-hunter-p11
    CMAKE_ARGS "CMAKE_CXX_FLAGS=-Wno-deprecated-copy"
)

hunter_config(
    spdlog
    VERSION 1.4.2
    CMAKE_ARGS "SPDLOG_FMT_EXTERNAL=NO"
)

cmake_minimum_required (VERSION 3.0)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

set(CMAKE_TOOLCHAIN_FILE
    "${CMAKE_SOURCE_DIR}/cmake/toolchain/cxx17.cmake"
    CACHE
    FILEPATH
    "Default toolchain"
    )

include(cmake/HunterGate.cmake)

HunterGate(
    URL "https://github.com/soramitsu/soramitsu-hunter/archive/e10a6614f59308033e125a3cb6e136bc28465546.tar.gz"
    SHA1 76e4c70323bdd1beebe1464d5de05f97c111d35a
    FILEPATH ${CMAKE_CURRENT_LIST_DIR}/cmake/HunterConfig.cmake
)

project (PolkadotRuntimeEnvironmentTests C CXX)

include(cmake/Dependencies.cmake)

# The test helpers in various languages
add_subdirectory(test/testers)

#add_custom_target(git_submodule_init ALL COMMAND git submodule update --init --recursive --remote)

# The Julia tester
##############################
# TBA

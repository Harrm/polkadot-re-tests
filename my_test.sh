#!/bin/bash

# METHODS: 'insert-and-delete' 'trie-root'

# don't pass
pk_branch_test2='/home/harrm/Projects/polkadot-re-tests/test/fixtures/pk_branch_test2.yaml'
random_state_trie_80='/home/harrm/Projects/polkadot-re-tests/test/fixtures/random_state_trie_80.yaml'

#pass
_1c1_trie="/home/harrm/Projects/polkadot-re-tests/test/fixtures/1c1_trie.yaml"
pk_branch_test='/home/harrm/Projects/polkadot-re-tests/test/fixtures/pk_branch_test.yaml'
scv_trie='/home/harrm/Projects/polkadot-re-tests/test/fixtures/scv_trie.yaml'
hex_1c1_trie='/home/harrm/Projects/polkadot-re-tests/test/fixtures/hex_1c1_trie.yaml'
hex_limit_trie='/home/harrm/Projects/polkadot-re-tests/test/fixtures/hex_limit_trie.yaml'
_10000_node_trie='/home/harrm/Projects/polkadot-re-tests/test/fixtures/10000-node-trie.yaml'

files=("$pk_branch_test2" "$random_state_trie_80" "$_1c1_trie" "$pk_branch_test" "$scv_trie")
wrong_files=("$pk_branch_test2" "$random_state_trie_80")

run_and_diff() {
  local METHOD=$1
  local FILE=$2

  local RUST_OUT=$(build/bin/usr/local/bin/rust_tester state-trie $METHOD --state-file $FILE)
  local KAGOME_OUT=$(build/bin/usr/local/bin/kagome_tester state-trie $METHOD --state-file $FILE)

  local CMP_RES=$(cmp <(echo "$RUST_OUT") <(echo "$KAGOME_OUT"))

  if [ "${CMP_RES}" ]
  then
    echo "$RUST_OUT" > rust_out.txt
    echo "$KAGOME_OUT" > kagome_out.txt
   #diff -y <(echo "$RUST_OUT") <()
  else
    echo "Identical"
  fi
}

run_and_diff 'insert-and-delete' "$pk_branch_test2"

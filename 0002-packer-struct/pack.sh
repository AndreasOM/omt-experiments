#!/bin/sh

#OMT_PACKER=omt-packer

OMT_PACKER=packer/target/debug/packer

#	PACK_OUT=$(ruby tools/madpack.rb Data App/data/base.omar @Data/data.paklist 2>&1)

# omt-packer Data base.omar @Data/data.paklist

${OMT_PACKER} pack --basepath testdata --output testdata/test1223334444.omar --paklist testdata/testdata122334444.paklist


# TEST
diff testdata/test1223334444.omar testdata/expected-result-test1223334444.omar && echo OK

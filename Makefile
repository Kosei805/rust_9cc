all: test

test:
	tmp=$RANDOM
	bash bin/test.sh $tmp $tmp

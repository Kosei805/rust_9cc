all: test

test:
	bash bin/input_test.sh

clean:
	rm -f tmp*

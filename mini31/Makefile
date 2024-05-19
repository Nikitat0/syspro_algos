RUNNABLE = build/main

IN = $(wildcard tests/*-input.txt)
ACT = $(IN:-input.txt=-actual.txt)
PASS = $(IN:-input.txt=.passed)


all: clean test

clean:
	@rm -f $(PASS)
	@rm -f $(ACT) $(EXE)

$(RUNNABLE): *.go
	go build -o $(RUNNABLE)

test: $(PASS)
	@echo "All tests passed"

$(PASS): %.passed: %-input.txt %-expected.txt $(RUNNABLE)
	@echo "Running test $*..."
	@rm -f $@
	$(RUNNABLE) < $*-input.txt > $*-actual.txt
	diff $*-expected.txt $*-actual.txt -Z
	@touch $@

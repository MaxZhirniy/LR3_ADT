CXX = g++
CXXFLAGS = -std=c++17

SOURCES = $(wildcard cpp/src/*.cpp)
HEADERS = $(wildcard cpp/include/*.hpp cpp/include/structures/*.hpp)

.PHONY: all rust clean

all: dbms rust

dbms: $(SOURCES) $(HEADERS)
	$(CXX) $(CXXFLAGS) -Icpp/include $(SOURCES) -o dbms

rust:
	cd rust && cargo build --release --offline
	cp rust/target/release/dbms-rust ./dbms-rust

clean:
	rm -f dbms dbms-rust
	rm -rf rust/target

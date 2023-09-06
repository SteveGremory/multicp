#include "mcp.hpp"

int main(int argc, char** argv) {
	if (argc != 3) {
		std::cerr << "Not enough args" << std::endl;
		exit(-1);
	}

	copy(argv[1], argv[2]);
}
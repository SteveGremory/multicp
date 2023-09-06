#include "mcp.hpp"
#include <filesystem>

void copy(const char* src, const char* dst) {
	// exit if there is nothing to copy
	if (!std::filesystem::exists(src)) {
		std::cerr << "The source file does NOT exist" << std::endl;
		exit(EXIT_FAILURE);
	}

	// Clonefile does not work if the dst exists and this also gives the user a
	// chance to re-evaluate their actions
	if (std::filesystem::exists(dst)) {
		std::cerr << "The destination file ALREADY exists" << std::endl;
		exit(EXIT_FAILURE);
	}

#ifdef __APPLE__

	if (clonefile(src, dst, (uint32_t)0) != 0) {
		std::cerr << "Failed to clone the file, performing a stack copy"
				  << std::endl;

		stack_buffer_copy(src, dst);
	}

#elif defined(__linux__)
	FILE *reader = NULL, *writer = NULL;

	reader = fopen(src, "rb");
	writer = fopen(dst, "wb");

	if (reader == NULL || writer == NULL) {
		perror("Could not open the reader/writer");
	}

	struct stat stat;
	size_t len = 0, ret = 0;
	if (fstat(reader->_fileno, &stat) == -1) {
		perror("Fstat has failed");
		exit(EXIT_FAILURE);
	}

	len = stat.st_size;

	do {
		ret = copy_file_range(reader->_fileno, NULL, writer->_fileno, NULL, len,
							  0);
		if (ret == -1) {
			perror("copy_file_range");
			exit(EXIT_FAILURE);
		}

		len -= ret;
	} while (len > 0 && ret > 0);

	fclose(reader);
	fclose(writer);
#else
	stack_buffer_copy(src, dst);

#endif
}

size_t stack_buffer_copy(FILE* reader, FILE* writer) {
// On windows, just use the pagesize
#ifdef _WIN32
	SYSTEM_INFO si;
	GetSystemInfo(&si);
	const size_t BUFFSIZE = si.dwPageSize;

// On Linux/macOS, compare the pagesize to the filesize to get the best possible
// caching and reading speeds
#else
	const size_t pagesize = sysconf(_SC_PAGESIZE);
	struct stat stat;

// The stat struct is different on Linux and macOS
#ifdef __APPLE__
	if (fstat(reader->_file, &stat) == -1) {
		perror("Fstat has failed");
		exit(EXIT_FAILURE);
	}

#else
	if (fstat(reader->_fileno, &stat) == -1) {
		perror("Fstat has failed");
		exit(EXIT_FAILURE);
	}
#endif

	const size_t scale = (size_t)std::ceil(stat.st_size / pagesize) % 16 + 1;
	const size_t BUFFSIZE = pagesize * scale;
#endif

	size_t total_bytes = 0;
	size_t bytes_read = 0;

	char buffer[BUFFSIZE];

	while ((bytes_read = fread(buffer, 1, BUFFSIZE, reader)) != 0) {
		if (ferror(reader)) {
			std::cerr << "Failed to read the source file" << std::endl;
			exit(EXIT_FAILURE);
		}
		if (fwrite(buffer, sizeof(char), bytes_read, writer) < 0) {
			std::cerr << "Failed to read the source file" << std::endl;
			exit(EXIT_FAILURE);
		}
		total_bytes += bytes_read;
	}

	return total_bytes;
}

size_t stack_buffer_copy(const char* src, const char* dst) {
	FILE *reader = NULL, *writer = NULL;

	reader = fopen(src, "rb");
	writer = fopen(dst, "wb");

	if (reader == NULL || writer == NULL) {
		perror("Could not open the reader/writer");
	}

	size_t copy_size = stack_buffer_copy(reader, writer);

	fclose(reader);
	fclose(writer);

	return copy_size;
}

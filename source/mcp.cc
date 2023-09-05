#include "mcp.hh"
#include <filesystem>

void copy(const char* src, const char* dst) {
#ifdef __APPLE__

	if (std::filesystem::exists(dst)) {
		std::filesystem::remove(dst);
	}

	if (clonefile(src, dst, (uint32_t)0) != 0) {
		perror("Failed to clone the file");
	}
#else

	FILE *reader = NULL, *writer = NULL;

	reader = fopen(src, "rb");
	writer = fopen(dst, "wb");

	if (reader == NULL || writer == NULL) {
		perror("Could not open the reader/writer");
	}

#ifdef __linux__
	struct stat  stat;
	size_t len = 0, ret = 0;
	if (fstat(reader->_fileno, &stat) == -1) {
		perror("Fstat has failed");	
		exit(EXIT_FAILURE);
	}

	len = stat.st_size;

	do {
		ret = copy_file_range(reader->_fileno, NULL, writer->_fileno, NULL, len, 0);
		if (ret == -1) {
			perror("copy_file_range");
			exit(EXIT_FAILURE);
		}

		len -= ret;
	} while (len > 0 && ret > 0);

#else
	stack_buffer_copy(reader, writer);
#endif

	fclose(reader);
	fclose(writer);
#endif
}

size_t stack_buffer_copy(FILE* reader, FILE* writer) {
#ifdef _WIN32
	SYSTEM_INFO si;
	GetSystemInfo(&si);

	const size_t BUFFSIZE = si.dwPageSize;
#else
	const size_t BUFFSIZE = sysconf(_SC_PAGESIZE);
#endif

	size_t total_bytes = 0;
	size_t bytes_read = 0;

	char* buffer = (char*)malloc(BUFFSIZE);

	while ((bytes_read = fread(buffer, 1, BUFFSIZE, reader)) != 0) {
		if (ferror(reader)) {
			throw std::runtime_error("Failed to read the source file");
		}
		if (fwrite(buffer, sizeof(char), bytes_read, writer) < 0) {
			throw std::runtime_error("Failed to read the source file");
		}
		total_bytes += bytes_read;
	}

	free(buffer);

	return total_bytes;
}

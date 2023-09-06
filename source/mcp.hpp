#pragma once

#include <cstdio>
#include <iostream>

#ifdef __linux__
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>
#endif

#ifdef __APPLE__
#include <sys/attr.h>
#include <sys/clonefile.h>
#include <unistd.h>
#endif

#ifdef _WIN32
#include <windows.h>
#endif

size_t stack_buffer_copy(FILE* reader, FILE* writer);

void copy(const char* src, const char* dst);

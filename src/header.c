#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

typedef int64_t integer;

#define DEF_CAPACITY (1 << 16)
typedef struct {
    int capacity;
    integer* memory;
} Stack;

typedef struct {
    int capacity;
    int front;
    int back;
    integer* memory;
} Queue;

#define DEF_BUFSIZ (1 << 17)
typedef struct {
   char buffer[DEF_BUFSIZ];
   int size;
   int off;
} IO;

union Storage {
    Stack stack;
    Queue queue;
};

void new_stack(Stack* st) {
    st->capacity = DEF_CAPACITY;
    st->memory = malloc(st->capacity * sizeof(integer));
}
void push_stack(Stack* st, int current_size, integer v) {
    st->memory[current_size++] = v;
    if (current_size >= st->capacity) {
	st->capacity *= 2;
	integer* new_memory = malloc(st->capacity * sizeof(integer));
	memcpy(new_memory, st->memory, current_size * sizeof(integer));
	free(st->memory);
	st->memory = new_memory;
    }
}

void new_queue(Queue* q) {
    q->front = 0;
    q->back = 0;
    q->capacity = DEF_CAPACITY;
    q->memory = malloc(q->capacity * sizeof(integer));
}
void extend_queue(Queue* q, int size) {
    int prev_capacity = q->capacity;
    integer* new_memory;
    q->capacity *= 2;
    new_memory = malloc(q->capacity * sizeof(integer));
    if (q->front <= q->back) {
        memcpy(new_memory, q->memory + q->front, (q->back - q->front) * sizeof(integer));
    } else {
        memcpy(new_memory, q->memory + q->front, (prev_capacity - q->front) * sizeof(integer));
        memcpy(new_memory + prev_capacity - q->front, q->memory, q->back * sizeof(integer));
    }
    free(q->memory);
    q->memory = new_memory;
    q->front = 0;
    q->back = size;
}
void push_queue(Queue* q, integer v, int newsize) {
    if (newsize == q->capacity) extend_queue(q, newsize - 1);
    q->memory[q->back++] = v;
    if (q->back == q->capacity) q->back = 0;
}
void push_queue_front(Queue* q, integer v) {
    if (q->front == 0) q->front = q->capacity;
    q->memory[--q->front] = v;
}
integer pop_queue(Queue* q) {
    integer v = q->memory[q->front++];
    if (q->front == q->capacity) q->front = 0;
    return v;
}

void new_stdout(IO* io) {
    io->off = 0;
}

void flush(IO* io) {
    write(1, io->buffer, io->off);
    io->off = 0;
}

void print_decimal(IO* io, integer v) {
    uint64_t w = v;
    char temp[24];
    int off = 24;
    int sign = v < 0;
    if (sign) w = -w;
    do {
	temp[--off] = w % 10 + '0';
    } while (w /= 10);
    if (sign) temp[--off] = '-';
    int len = 24 - off;
    if (io->off + len > DEF_BUFSIZ) flush(io);
    memcpy(io->buffer + io->off, temp + off, len);
    io->off += len;
}

void print_utf8(IO* io, integer codepoint) {
    if (codepoint < 0x80) {
	if (io->off + 1 > DEF_BUFSIZ) flush(io);
	io->buffer[io->off++] = codepoint;
    } else if (codepoint < 0x800) {
	if (io->off + 2 > DEF_BUFSIZ) flush(io);
	io->buffer[io->off++] = 0xC0 | (codepoint >> 6);
	io->buffer[io->off++] = 0x80 | (codepoint & 0x3F);
    } else if (codepoint < 0x10000) {
	if (io->off + 3 > DEF_BUFSIZ) flush(io);
	io->buffer[io->off++] = 0xE0 | (codepoint >> 12);
	io->buffer[io->off++] = 0x80 | ((codepoint >> 6) & 0x3F);
	io->buffer[io->off++] = 0x80 | (codepoint & 0x3F);
    } else {
	if (io->off + 4 > DEF_BUFSIZ) flush(io);
	io->buffer[io->off++] = 0xF0 | (codepoint >> 18);
	io->buffer[io->off++] = 0x80 | ((codepoint >> 12) & 0x3F);
	io->buffer[io->off++] = 0x80 | ((codepoint >> 6) & 0x3F);
	io->buffer[io->off++] = 0x80 | (codepoint & 0x3F);
    }
}

void new_stdin(IO* io) {
    io->size = DEF_BUFSIZ;
    io->off = io->size;
}

signed char get_or_refill(IO* io) {
    if (!io->size) return -1;
    if (io->off >= io->size) {
	    io->off = 0;
	    io->size = read(0, io->buffer, DEF_BUFSIZ);
        if (!io->size) return -1;
    }
    return io->buffer[io->off++];
}

integer scan_decimal(IO* io) {
    uint64_t v = 0;
    signed char c = get_or_refill(io);
    if (c == -1) return -1;
    int sign = c == '-';
    if (sign) c = get_or_refill(io);
    while ('0' <= c && c <= '9') {
        v = v * 10 + c - '0';
        c = get_or_refill(io);
    }
    if (sign) v = -v;
    return v;
}

integer scan_utf8(IO* io) {
    signed char c = get_or_refill(io);
    if (c == -1) return -1;
    integer v = (unsigned char) c;
    if (!(v & 0x80)) return v;
    else if (!(v & 0x20)) {
	v &= 0x1F;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
    } else if (!(v & 0x10)) {
	v &= 0x0F;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
    } else {
	v &= 0x07;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
	v <<= 6;
	v |= get_or_refill(io) & 0x3F;
    }
    return v;
}

#define PUSHS(i, j) push_stack(&storage[i].stack, size[i]++, v ## j)
#define POPS(i) storage[i].stack.memory[--size[i]]
#define PUSHQ(i, j) push_queue(&storage[i].queue, v ## j, ++size[i])
#define POPQ(i) (size[i]--, pop_queue(&storage[i].queue))
#define PRINTD(i) print_decimal(&output, v ## i)
#define PRINTU(i) print_utf8(&output, v ## i)
#define SCAND scan_decimal(&input)
#define SCANU scan_utf8(&input)
#define HALTS(i) return (flush(&output), size[i] ? storage[i].stack.memory[--size[i]] : 0)
#define HALTQ(i) return (flush(&output), size[i] ? pop_queue(&storage[i].queue) : 0)
#define JSL(i, n, j, k) if (size[i] < n) goto B ## j; else goto B ## k;
#define JNZQ(i, j, k) if (POPQ(i)) goto B ## j; else goto B ## k;
#define JNZS(i, j, k) if (POPS(i)) goto B ## j; else goto B ## k;

int main() {
    IO input, output;
    new_stdin(&input);
    new_stdout(&output);
    union Storage storage[28];
    for (int i = 0; i < 28; ++i) {
	if (i == 21) new_queue(&storage[i].queue);
	else new_stack(&storage[i].stack);
    }
    int size[28] = {};
    integer local0, local1;
    int select = 0, tmp;

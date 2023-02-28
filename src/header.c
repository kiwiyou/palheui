#include <stdlib.h>
#include <string.h>
#include <unistd.h>

typedef int integer;

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
    char temp[16];
    int off = 16;
    int sign = v < 0;
    if (sign) v = -v;
    do {
	temp[--off] = v % 10 + '0';
    } while (v /= 10);
    if (sign) temp[--off] = '-';
    int len = 16 - off;
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
    }
    return io->buffer[io->off++];
}

integer scan_decimal(IO* io) {
    integer v = 0;
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

#define NOP
#define HALT flush(&output); return local0
#define ADD local0 += local1
#define MULTIPLY local0 *= local1
#define SUBTRACT local0 -= local1
#define DIVIDE local0 /= local1
#define REMAINDER local0 %= local1
#define PRINT_DECIMAL print_decimal(&output, local0)
#define PRINT_UNICODE print_utf8(&output, local0)
#define SCAN_DECIMAL local0 = scan_decimal(&input)
#define SCAN_UNICODE local0 = scan_utf8(&input)
#define SELECT(n) select = n
#define COMPARE local0 = (local0 >= local1)
#define JUMP_NOT_EQUAL_ZERO(label) if (local0 != 0) goto label
#define STACK_POP0 local0 = storage[select].stack.memory[--size[select]]
#define STACK_POP1 local1 = storage[select].stack.memory[--size[select]]
#define STACK_PUSH0 push_stack(&storage[select].stack, size[select]++, local0)
#define STACK_PUSH1 push_stack(&storage[select].stack, size[select]++, local1)
#define STACK_PUSH(v) push_stack(&storage[select].stack, size[select]++, v)
#define QUEUE_POP0 size[select]--; local0 = pop_queue(&storage[select].queue)
#define QUEUE_POP1 size[select]--; local1 = pop_queue(&storage[select].queue)
#define QUEUE_PUSH0 size[select]++; push_queue(&storage[select].queue, local0, size[select])
#define QUEUE_PUSH1 size[select]++; push_queue(&storage[select].queue, local1, size[select])
#define QUEUE_PUSH(v) size[select]++; push_queue(&storage[select].queue, v, size[select])
#define PUSH0_TO(n) tmp = select; select = n; if (select == 21) { QUEUE_PUSH0; } else { STACK_PUSH0; } select = tmp
#define PUSH_FRONT_0 size[select]++; push_queue_front(&storage[select].queue, local0)
#define PUSH_FRONT_1 size[select]++; push_queue_front(&storage[select].queue, local1)
#define JUMP_SIZE_NOT_LESS(n, label) if (size[select] >= n) goto label
#define JUMP(label) goto label

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

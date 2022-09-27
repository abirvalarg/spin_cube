#ifdef LINUX
#include <asm-generic/ioctls.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <termios.h>
#endif
#include <stdio.h>

struct TermSize {
    unsigned width;
    unsigned height;
};

struct TermSize get_term_size()
{
    #ifdef LINUX
    struct winsize size;
    ioctl(STDOUT_FILENO, TIOCGWINSZ, &size);
    return (struct TermSize){
        // .width = size.ws_row,
        // .height = size.ws_col
        .width = size.ws_col,
        .height = size.ws_row
    };
    #endif
}

void flush_stdout()
{
    fflush(stdout);
}

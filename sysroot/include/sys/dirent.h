#include <sys/types.h>

typedef struct _DIR DIR;

struct dirent {
    ino_t  d_ino;
    char   d_name[32];
};

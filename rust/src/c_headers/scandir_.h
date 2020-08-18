typedef struct {
    const ignores *ig;
    const char *base_path;
    size_t base_path_len;
    const char *path_start;
} scandir_baton_t;

typedef int (*filter_fp)(const char *path, const struct dirent *, void *);

int ag_scandir(const char *dirname, struct dirent ***namelist, filter_fp f, void *baton);

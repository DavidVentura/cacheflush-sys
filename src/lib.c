volatile void clear_cache(void* start, void* end) {
    __builtin___clear_cache(start, end);
}

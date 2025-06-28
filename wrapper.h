#include <nds.h>
#ifdef ARM7
#include <nds/arm7/dldi.h>
#include <maxmod7.h>
#include <dswifi7.h>
#elif ARM9
#include <dirent.h>
#include <sys/stat.h>
#include <filesystem.h>
#include <nds/arm9/dldi.h>
#include <maxmod9.h>
#include <dswifi9.h>
#endif

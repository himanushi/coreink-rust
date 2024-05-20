#define ESP_PLATFORM

#define CONFIG_FREERTOS_NUMBER_OF_CORES 1
#define CONFIG_FREERTOS_MAX_TASK_NAME_LEN 16
#define CONFIG_FREERTOS_TASK_NOTIFICATION_ARRAY_ENTRIES 1

#include <memory>
#include "clibs/M5GFX/src/M5GFX.h"
#include "clibs/M5Unified/src/M5Unified.h"

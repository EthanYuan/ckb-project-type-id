#include "ckb-c-stdlib/ckb_type_id.h"

int validate_type_id(const uint8_t type_id[32]) {
    return ckb_validate_type_id(type_id);
}

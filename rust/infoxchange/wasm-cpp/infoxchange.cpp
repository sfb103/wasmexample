#include <atomic>
#include <iostream>
#include "wit_bindgen/infoxchange.h"

static auto ID = new std::atomic<int>(-1);
int id_data = 0;

// Exported Functions from `wasmexample:infoxchange/id-holder`
bool exports_wasmexample_infoxchange_id_holder_set_id(int32_t id, infoxchange_string_t *err){
    std::cout << "set_id() setting id: " << id << std::endl;
    ID->store(id);
    return true;
}

bool exports_wasmexample_infoxchange_id_holder_get_id(int32_t *ret, infoxchange_string_t *err){
    id_data = ID->load();
    ret = &id_data;
    return true;
}

// Exported Functions from `wasmexample:infoxchange/worker`
bool exports_wasmexample_infoxchange_worker_do_work(void){
    std::cout << "do_work() hello from your cpp wasm component!" << std::endl;
    int id = ID->load();
    infoxchange_string_t* result;    
    if( id < 0 ){
        std::cout << "do_work() id: " << id << " <= 0, setting status to Offline" << std::endl;
        wasmexample_infoxchange_status_holder_set_status(
            WASMEXAMPLE_INFOXCHANGE_STATUS_HOLDER_STATUS_OFFLINE,
            result);
        return true; // Keep going. 
    } else {
        std::cout << "do_work() id: " << id << " > 0, setting status to Online" << std::endl;
        wasmexample_infoxchange_status_holder_set_status(
            WASMEXAMPLE_INFOXCHANGE_STATUS_HOLDER_STATUS_ONLINE,
            result);
        return false; // Done.
    }
}
#include "mod_world.h"
#include <stdio.h>

void exports_mod_world_init(void) { printf("C: init()\n"); }

void exports_mod_world_actor_init(mod_world_borrow_actor_t a, float x,
                                  float y) {
  printf("C: actor_init\n");
  spaghettikart_module_iactor_method_actor_outside_init(a, x, y);
}

void exports_mod_world_actor_update(mod_world_borrow_actor_t a) {
  spaghettikart_module_iactor_method_actor_update(a);
}

void exports_mod_world_actor_render(mod_world_borrow_actor_t a) {
  spaghettikart_module_iactor_method_actor_render(a);
}

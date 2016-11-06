#include <Elementary.h>

typedef void (*Request_Login_Cb)(void* data, const char* login, const char* pass);

struct Login {
  Request_Login_Cb cb;
  void *data;
  Eo* object;
  Eo* username;
  Eo* pass;
};

void* login_new(Request_Login_Cb request_login_cb, void* data);
void login_success(Eina_Bool b);

void efl_init();
void efl_run();



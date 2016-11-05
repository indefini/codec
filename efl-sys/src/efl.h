#include <Elementary.h>

typedef void (*Request_Login_Cb)(void* data, const char* login, const char* pass);

struct Login {
  Request_Login_Cb cb;
  void *data;
  Eo* object;
};

void* login_new(Request_Login_Cb request_login_cb, void* data);
void login_success(Eina_Bool b);


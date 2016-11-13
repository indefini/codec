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

struct Loading {
  Eo* object;
  Eo* label;
};

struct Chat {
  Eo* object;
  Eo* box;
  Eina_List *lines;
};

struct Ui {
  struct Login *login;
  struct Loading *loading;
  struct Chat* chat;
};

struct Ui* ui_new(Request_Login_Cb request_login_cb, void* data);

void efl_init();
void efl_run();

void login_visible_set(Eina_Bool b);
void loading_visible_set(Eina_Bool b);

void chat_visible_set(Eina_Bool b);
void chat_text_add(const char *user, const char *time, const char *message);


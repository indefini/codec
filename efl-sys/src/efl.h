#include <Elementary.h>

typedef void (*Request_Login_Cb)(void* data, const char* login, const char* pass);
typedef void (*Key_Press_Cb)(void* data, const char* modifier, const char* key);

struct Login {
  Request_Login_Cb cb;
  void *data;
  Eo* object;
  Eo* username;
  Eo* pass;
};

void* login_new(Request_Login_Cb request_login_cb, void* data);

struct Loading {
  Eo* object;
  Eo* label;
};

struct Room {
  Eo* object;
  Eo* box;
  Eo* title;
  Eina_List *lines;
};

struct Chat {
  Eo* object;
  Eina_Hash* rooms;
  struct Room *room_current;
};

struct Notify {
  Eo* object;
  Eo* box;
  Eo* room;
  Eo* user;
  Eo* message;
  int state;
};

struct RoomSelector
{
  Eo* object;
};

struct Ui {
  struct Login *login;
  struct Loading *loading;
  struct Chat* chat;
  struct Notify* notify;

  Evas_Object* win;
};

struct Ui* ui_new(
    Request_Login_Cb request_login_cb,
    Key_Press_Cb key_press_cb,
    void* data);

void efl_init();
void efl_run();

void login_visible_set(Eina_Bool b);
void loading_visible_set(Eina_Bool b);

void chat_visible_set(Eina_Bool b);

void notify_add(const char *room, const char* user, const char* message);

void room_new(const char *id);
void room_set(const char *id);
void room_text_add(const char *room_id, const char *user, const char *time, const char *message);
void room_title_set(const char *id, const char* title);


#include "efl.h"
#include <stdio.h>

void efl_init()
{
  elm_init(0,0);
}

void efl_run()
{
  elm_run();
  elm_shutdown();
}

void kexit()
{
  elm_exit();
}

static Eo* _win = NULL;

static void
_window_del(void *data, Evas_Object* o, void* event_info)
{
  elm_exit();
}

Eo* window_get_or_create()
{
  if (_win == NULL) {
    _win = elm_win_util_standard_add("codec", "codec");
    elm_win_autodel_set(_win, EINA_TRUE);
    evas_object_smart_callback_add(_win, "delete,request", _window_del, NULL);
  }

  return _win;
}

static void
login_entry_activated_cb(void *data EINA_UNUSED, Evas_Object *obj, void *event_info EINA_UNUSED)
{
     printf("entry activated login : %s\n", elm_entry_entry_get(obj));
     struct Login* log = data;
     elm_object_focus_set(log->pass, EINA_TRUE);
}


static void
password_entry_activated_cb(void *data EINA_UNUSED, Evas_Object *obj, void *event_info EINA_UNUSED)
{
     printf("entry activated Password : %s\n", elm_entry_entry_get(obj));
     struct Login* log = data;
     log->cb(
         log->data,
         elm_entry_entry_get(log->username),
         elm_entry_entry_get(obj));
}

static void
show_password_check_changed_cb(void *data, Evas_Object *obj, void *event_info EINA_UNUSED)
{
  Evas_Object *en = (Evas_Object *)data;
  Eina_Bool state = elm_check_state_get(obj);

  if (state)
  {
    printf(" * Show Password...\n");
    elm_object_text_set(obj, "Hide Password");
    elm_entry_password_set(en, EINA_FALSE);
  }
  else
  {
    printf(" * Hide Password...\n");
    elm_object_text_set(obj, "Show Password");
    elm_entry_password_set(en, EINA_TRUE);
  }
}

void* login_new(Request_Login_Cb request_login_cb, void* data) {

  Eo *win, *bx, *en, *ck;
  struct Login *log = calloc(1, sizeof(*log));
  
  win = window_get_or_create();

  bx = elm_box_add(win);
  evas_object_size_hint_weight_set(bx, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  elm_win_resize_object_add(win, bx);
  evas_object_show(bx);

  en = elm_entry_add(bx);
  elm_entry_single_line_set(en, EINA_TRUE);
  elm_entry_scrollable_set(en, EINA_TRUE);
  elm_object_part_text_set(en, "elm.guide", "Enter Your Login");
  evas_object_size_hint_weight_set(en, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(en, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_pack_end(bx, en);
  evas_object_show(en);

  evas_object_smart_callback_add(
      en,
      "activated",
      login_entry_activated_cb,
      log);

  log->username = en;

  en = elm_entry_add(bx);
  elm_entry_single_line_set(en, EINA_TRUE);
  elm_entry_scrollable_set(en, EINA_TRUE);
  elm_entry_password_set(en, EINA_TRUE);
  elm_object_part_text_set(en, "elm.guide", "Enter Your Password");
  evas_object_size_hint_weight_set(en, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(en, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_pack_end(bx, en);
  evas_object_show(en);

  evas_object_smart_callback_add(
      en,
      "activated",
      password_entry_activated_cb,
      log);

  log->pass = en;

  ck = elm_check_add(bx);
  elm_object_text_set(ck, "Show Password");
  evas_object_smart_callback_add(
      ck,
      "changed",
      show_password_check_changed_cb,
      en);

  elm_box_pack_end(bx, ck);
  evas_object_show(ck);

  evas_object_resize(win, 300, 100);
  evas_object_show(win);

  log->cb = request_login_cb;
  log->data = data;
  log->object = bx;

  return log;
}

void login_success(Eina_Bool b) {
  printf("success : %d \n", b);
}


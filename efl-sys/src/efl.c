#include "efl.h"
#include <stdio.h>

#define LOGIN_ENTRY_SIZE_MIN 50
#define LOGIN_ENTRY_SIZE 200

#define RECT_SIZE 32

struct Ui* _ui = NULL;

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

struct Loading* loading_new(Evas_Object* win)
{
  struct Loading* loading = calloc(1, sizeof *loading);

  Evas_Object *box, *rect, *label;

  box = elm_box_add(win);
  ///evas_object_show(box);

  rect = evas_object_rectangle_add(evas_object_evas_get(win));
  elm_box_pack_end(box, rect);
  evas_object_show(rect);
  evas_object_color_set(rect, 0, 0, 0, 255);

  evas_object_size_hint_min_set(rect, RECT_SIZE, RECT_SIZE);
  evas_object_size_hint_max_set(rect, RECT_SIZE, RECT_SIZE);

  Elm_Transit *trans;

  trans = elm_transit_add();
  elm_transit_object_add(trans, rect);
  elm_transit_repeat_times_set(trans, -1);

  elm_transit_effect_rotation_add(trans, 0.0, 360.0);

  elm_transit_duration_set(trans, 2.0);
  elm_transit_go(trans);

  label = elm_label_add(win);
  elm_object_text_set(label, "<style=outline_soft_shadow outline_color=#1a0a1a font_size=10>login</>");
  evas_object_show(label);
  elm_box_pack_end(box, label);

  loading->object = box;
  loading->label = label;

  return loading;
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

static void
_changed_size_hints(void* data, Evas* e, Evas_Object* o, void *event_info)
{
  Evas_Coord minw, minh, maxw, maxh;
  evas_object_size_hint_min_get(o, &minw, &minh);
  evas_object_size_hint_max_get(o, &maxw, &maxh);
  minw = minw > LOGIN_ENTRY_SIZE_MIN ? minw : LOGIN_ENTRY_SIZE_MIN;

  evas_object_size_hint_min_set(data, minw, minh);
}

void* login_new(Request_Login_Cb request_login_cb, void* data) {

  Eo *win, *table, *bx, *grid, *en, *ck;
  struct Login *log = calloc(1, sizeof(*log));
  
  win = window_get_or_create();

  table = elm_table_add(win);
  evas_object_size_hint_weight_set(table, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  elm_win_resize_object_add(win,table);
  evas_object_show(table);

  grid = elm_grid_add(win);
  evas_object_size_hint_weight_set(grid, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(grid, EVAS_HINT_FILL, 0.5);
  evas_object_size_hint_min_set(grid, LOGIN_ENTRY_SIZE_MIN, -1);
  evas_object_size_hint_max_set(grid, LOGIN_ENTRY_SIZE, -1);
  elm_table_pack(table, grid, 0, 0, 1, 1);
  evas_object_show(grid);

  bx = elm_box_add(win);
  evas_object_size_hint_weight_set(bx, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(bx, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_grid_pack(grid, bx, 0, 0, 100, 100);
  evas_object_show(bx);

  en = elm_entry_add(bx);
  elm_entry_single_line_set(en, EINA_TRUE);
  elm_entry_scrollable_set(en, EINA_TRUE);
  elm_object_part_text_set(en, "elm.guide", "Enter Your Login");
  evas_object_size_hint_weight_set(en, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(en, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_pack_end(bx, en);
  evas_object_show(en);

  evas_object_event_callback_add(bx, EVAS_CALLBACK_CHANGED_SIZE_HINTS, _changed_size_hints, grid);

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

  evas_object_resize(win, 400, 200);
  evas_object_show(win);

  log->cb = request_login_cb;
  log->data = data;
  log->object = bx;

  return log;
}

void login_success(Eina_Bool b) {
  printf("success : %d \n", b);
  if (b) {
    evas_object_hide(_ui->login->object);
    //print
  }
}

struct Chat* chat_new(Evas_Object* win)
{
  struct Chat* chat = calloc(1, sizeof *chat);

  Evas_Object* bx = elm_box_add(win);
  evas_object_size_hint_weight_set(bx, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  elm_win_resize_object_add(win, bx);
  //evas_object_show(bx);

  Evas_Object* label = elm_label_add(win);
  elm_object_text_set(label, "chat room.........");
  elm_box_pack_end(bx, label);
  evas_object_show(label);

  chat->object = bx;
  chat->box = bx;

  return chat;
}

struct Ui* ui_new(
    Request_Login_Cb request_login_cb,
    void* data)
{
  struct Ui *ui = calloc(1, sizeof *ui);
  _ui = ui;
  Eo* win = window_get_or_create();
  ui->login = login_new(request_login_cb, data);
  ui->loading = loading_new(win);
  ui->chat = chat_new(win);

  return ui;
}

static void
_visible_set(Evas_Object* o, Eina_Bool b)
{
  if (b)
  evas_object_show(o);
  else
  evas_object_hide(o);
}

void login_visible_set(Eina_Bool b)
{
  _visible_set(_ui->login->object, b);
}

void loading_visible_set(Eina_Bool b)
{
  _visible_set(_ui->loading->object, b);
}

void chat_visible_set(Eina_Bool b)
{
  _visible_set(_ui->chat->object, b);
}

void loading_text_set(const char* text)
{
  elm_object_text_set(_ui->loading->label, text);
}


//void anim_add(void (*cb)(void*, void*), void* user_data)
//{
  //ecore_animator_add(cb, user_data);
//}

void chat_text_add(const char *user, const char *time, const char *message)
{
  Eo* bx_parent = _ui->chat->box;

  Eo* bx_msg = elm_box_add(bx_parent);
  evas_object_size_hint_weight_set(bx_msg, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(bx_msg, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_horizontal_set(bx_msg,  EINA_TRUE);
  elm_box_padding_set(bx_msg, 4, 4);
  elm_box_pack_end(bx_parent, bx_msg);
  evas_object_show(bx_msg);

  Eo* label = elm_label_add(bx_msg);
  elm_object_text_set(label, "  ");
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 0.5, 0.5);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  label = elm_label_add(bx_msg);
  elm_object_text_set(label, user);
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 1, 0.5);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  Eo* sp = elm_separator_add(bx_msg);
  elm_box_pack_end(bx_msg, sp);
  evas_object_show(sp);

  label = elm_label_add(bx_msg);
  evas_object_size_hint_weight_set(label, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 0, 0.5);
  elm_object_text_set(label, message);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  sp = elm_separator_add(bx_msg);
  elm_box_pack_end(bx_msg, sp);
  evas_object_show(sp);

  label = elm_label_add(bx_msg);
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 1, 0.5);
  elm_object_text_set(label, time);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  label = elm_label_add(bx_msg);
  elm_object_text_set(label, "  ");
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 0.5, 0.5);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

}

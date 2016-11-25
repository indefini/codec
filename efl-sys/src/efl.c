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
//TODO remove
static Key_Press_Cb _key_press_cb = NULL;
static void* _core_data = NULL;

static void
_window_del(void *data, Evas_Object* o, void* event_info)
{
  elm_exit();
}

static Eina_Bool
_elm_event_win(void *data, Evas_Object* o, Evas_Object* src, Evas_Callback_Type type, void* event_info)
{
  if (type == EVAS_CALLBACK_KEY_DOWN) {
    Evas_Event_Key_Down *ev = event_info;
    if (evas_key_modifier_is_set(ev->modifiers, "Control") &&
        !strcmp(ev->key, "Tab")) {
        printf("Key Down TABBBB : %s, TODO : use _ui to do something\n", ev->key);
        if (_key_press_cb && _core_data) {
          _key_press_cb(_core_data, "Control", ev->key);
        }
    }
  }
  return EINA_FALSE;
}

Eo* window_get_or_create()
{
  if (_win) {
    return _win;
  }

    _win = elm_win_util_standard_add("codec", "codec");
    elm_win_autodel_set(_win, EINA_TRUE);
    evas_object_smart_callback_add(_win, "delete,request", _window_del, NULL);

  elm_object_event_callback_add(_win, _elm_event_win, NULL);

  if (!evas_object_key_grab(_win, "Escape", 0, 0, EINA_TRUE)){
    printf("could not grab key. \n");
  }

  Evas* e = evas_object_evas_get(_win);
  Evas_Modifier_Mask modifier;
  modifier = evas_key_modifier_mask_get(e, "Control");
  if (!evas_object_key_grab(_win, "Tab", modifier, 0, EINA_TRUE)) {
    printf("could not grab key. \n");
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

static void
_room_free(void *data)
{
  free(data);
}

struct Chat*
chat_new(Evas_Object* win)
{
  struct Chat *chat = calloc(1, sizeof *chat);

  Evas_Object *bxwin, *bx, *label, *scroller, *en;

  bxwin = elm_table_add(win);
  evas_object_size_hint_weight_set(bxwin, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  //elm_box_homogeneous_set(bxwin, EINA_FALSE);
  elm_win_resize_object_add(win, bxwin);

  chat->object = bxwin;
  chat->rooms = eina_hash_string_superfast_new(_room_free);

  return chat;
}

static void
_notify_timeout(void *data EINA_UNUSED, Evas_Object *obj EINA_UNUSED, void *event_info EINA_UNUSED)
{
 printf("end of timeout\n"); 
}

struct Notify* notify_new(Evas_Object* win)
{
  struct Notify* n = calloc(1, sizeof *n);

  Evas_Object *notify, *bx, *lb;

  notify = elm_notify_add(win);
  evas_object_size_hint_weight_set(notify, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  elm_notify_align_set(notify, 1.0, 0.0);
  elm_notify_timeout_set(notify, 10.0);

  evas_object_smart_callback_add(notify, "timeout", _notify_timeout, NULL);

  bx = elm_box_add(win);
  elm_object_content_set(notify, bx);
  elm_box_horizontal_set(bx, EINA_TRUE);
  evas_object_show(bx);

  lb = elm_label_add(win);
  elm_box_pack_end(bx, lb);
  evas_object_show(lb);
  n->room = lb;

  Eo* sp = elm_separator_add(bx);
  elm_box_pack_end(bx, sp);
  evas_object_show(sp);

  lb = elm_label_add(win);
  elm_box_pack_end(bx, lb);
  evas_object_show(lb);
  n->user = lb;

  sp = elm_separator_add(bx);
  elm_box_pack_end(bx, sp);
  evas_object_show(sp);

  lb = elm_label_add(win);
  elm_box_pack_end(bx, lb);
  evas_object_show(lb);
  n->message = lb;

  n->object = notify;
  n->box = bx;

  return n;
}

struct Ui* ui_new(
    Request_Login_Cb request_login_cb,
    Key_Press_Cb key_press_cb,
    void* data)
{
  struct Ui *ui = calloc(1, sizeof *ui);
  _ui = ui;
  _key_press_cb = key_press_cb;
  _core_data = data;

  Eo* win = window_get_or_create();
  ui->win = win;
  ui->login = login_new(request_login_cb, data);
  ui->loading = loading_new(win);
  ui->chat = chat_new(win);
  ui->notify = notify_new(win);

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

void room_text_add(
    const char* room_id,
    const char *user,
    const char *time,
    const char *message)
{
  struct Room* room = eina_hash_find(_ui->chat->rooms, room_id);
  if (!room) {
    printf("could not find room..., should add it? return for the moment\n");
    return;
  }

  Eo* bx_parent = room->box;

  Eo* bx_msg = elm_box_add(bx_parent);
  evas_object_size_hint_weight_set(bx_msg, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(bx_msg, EVAS_HINT_FILL, 1.0);
  elm_box_horizontal_set(bx_msg,  EINA_TRUE);
  elm_box_padding_set(bx_msg, 4, 4);
  elm_box_pack_end(bx_parent, bx_msg);
  evas_object_show(bx_msg);

  Eo* label = elm_label_add(bx_msg);
  elm_object_text_set(label, "  ");
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 0.5, EVAS_HINT_FILL);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  label = elm_label_add(bx_msg);
  elm_object_text_set(label, user);
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 1, EVAS_HINT_FILL);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  Eo* sp = elm_separator_add(bx_msg);
  elm_box_pack_end(bx_msg, sp);
  evas_object_show(sp);

  label = elm_entry_add(bx_msg);
  elm_entry_editable_set(label, EINA_FALSE);
  evas_object_size_hint_weight_set(label, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(label, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_object_text_set(label, message);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  sp = elm_separator_add(bx_msg);
  elm_box_pack_end(bx_msg, sp);
  evas_object_show(sp);

  label = elm_label_add(bx_msg);
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 1, EVAS_HINT_FILL);
  elm_object_text_set(label, time);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

  label = elm_label_add(bx_msg);
  elm_object_text_set(label, "  ");
  evas_object_size_hint_weight_set(label, 0, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(label, 0.5, EVAS_HINT_FILL);
  elm_box_pack_end(bx_msg, label);
  evas_object_show(label);

}

void notify_add(const char *room, const char* user, const char* message)
{
  struct Notify* notify = _ui->notify;

  elm_object_text_set(notify->room, room);
  elm_object_text_set(notify->user, user);
  evas_object_show(notify->object);

  if (strlen(message) > 50) {
    elm_label_line_wrap_set(notify->message, ELM_WRAP_MIXED);
    elm_label_wrap_width_set(notify->message, 200);
  }
  else {
    elm_label_line_wrap_set(notify->message, ELM_WRAP_NONE);
  }

  elm_object_text_set(notify->message, message);
}

static struct Room*
_room_new(Evas_Object* win)
{
  Evas_Object* chat_parent = _ui->chat->object;

  struct Room *room = calloc(1, sizeof *room);

  Evas_Object *bxwin, *bx, *label, *scroller, *en;

  bxwin = elm_box_add(win);
  evas_object_size_hint_weight_set(bxwin, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(bxwin, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_homogeneous_set(bxwin, EINA_FALSE);
  elm_table_pack(chat_parent, bxwin, 0, 0, 1, 1);

  label = elm_entry_add(win);
  elm_entry_editable_set(label,EINA_FALSE);
  elm_object_text_set(label, "chat room.........");
  evas_object_size_hint_weight_set(label, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(label, EVAS_HINT_FILL, 0);
  elm_box_pack_end(bxwin, label);
  evas_object_show(label);

  room->object = bxwin;
  room->title = label;

  scroller = elm_scroller_add(win);
  evas_object_size_hint_weight_set(scroller, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(scroller, EVAS_HINT_FILL, EVAS_HINT_FILL);
  evas_object_show(scroller);
  elm_scroller_gravity_set(scroller, 0, 1.0);
  elm_box_pack_end(bxwin, scroller);

  bx = elm_box_add(win);
  elm_box_align_set(bx, 0.5, 0);
  evas_object_size_hint_weight_set(bx, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(bx, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_object_content_set(scroller, bx);
  evas_object_show(bx);
  room->box = bx;

  //input box
  bx = elm_box_add(win);
  evas_object_size_hint_weight_set(bx, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(bx, EVAS_HINT_FILL, 1.0);
  elm_box_pack_end(bxwin, bx);
  evas_object_show(bx);

  en = elm_entry_add(win);
  evas_object_size_hint_weight_set(en, EVAS_HINT_EXPAND, 0);
  evas_object_size_hint_align_set(en, EVAS_HINT_FILL, EVAS_HINT_FILL);
  elm_box_pack_end(bx, en);
  evas_object_show(en);

  return room;
}

void room_new(const char *id)
{
  eina_hash_add(_ui->chat->rooms, id, _room_new(_ui->win));
}

void room_set(const char *id)
{
  struct Chat *chat = _ui->chat;

  if (chat->room_current) {
    evas_object_hide(chat->room_current->object);
  }

  struct Room* room = eina_hash_find(chat->rooms, id);
  if (!room) {
    printf("could not find room..., should add it? return for the moment\n");
    return;
  }

  chat->room_current = room;

  evas_object_show(chat->room_current->object);

}

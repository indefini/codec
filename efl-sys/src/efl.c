#include "efl.h"
#include <stdio.h>

#define LOGIN_ENTRY_SIZE_MIN 50
#define LOGIN_ENTRY_SIZE 200

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

Evas_Object* loading_new(Evas_Object* win)
{
  Evas_Object *rect;
  rect = evas_object_rectangle_add(evas_object_evas_get(win));
  evas_object_size_hint_weight_set(rect, EVAS_HINT_EXPAND, EVAS_HINT_EXPAND);
  evas_object_size_hint_align_set(rect, EVAS_HINT_FILL, EVAS_HINT_FILL);
  evas_object_resize(rect, 100, 100);
  evas_object_show(rect);

   Elm_Transit *trans;

   trans = elm_transit_add();
   elm_transit_object_add(trans, rect);
   elm_transit_repeat_times_set(trans, -1);

   elm_transit_effect_rotation_add(trans, 0.0, 360.0);

   elm_transit_duration_set(trans, 1.0);
   elm_transit_go(trans);
   return rect;
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

  evas_object_resize(win, 700, 400);
  evas_object_show(win);

  log->cb = request_login_cb;
  log->data = data;
  log->object = bx;

  elm_box_pack_end(bx, loading_new(win));

  return log;
}

void login_success(Eina_Bool b) {
  printf("success : %d \n", b);
}


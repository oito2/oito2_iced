🌐 [English](../../en/reference/components.md) | **Português** | 🏠 [Índice](../index.md)

---

# Componentes

Todas as páginas de componentes do catálogo do showcase (`apps/showcase/src/catalog.rs`) e os itens Rust por trás delas. Os caminhos são relativos ao crate indicado no título da seção; `widget::` significa `oito2_iced::widget::`. As outras páginas do showcase estão listadas em [Páginas do showcase](./showcase-pages.md).

## Componentes (`oito2_iced`)

| Página do showcase | Módulo | Itens principais |
|---|---|---|
| AngleSlider | `widget::angle_slider` | `angle_slider`, `angle_text` |
| AnimatedIcon | `icon::animated` | `icon`, `Mode`, `OK`, `HEART`, `STAR`, `COMPASS`, `NO_NETWORK`… |
| Autocomplete | `widget::autocomplete` | `autocomplete` |
| Avatar | `widget::avatar` | `avatar`, `group`, `Status`, `Shape` |
| Banner | `widget::info` | `banner`, `banner_tile` |
| BorderContainer | `widget::containers` | `border_container` |
| Button | `widget::button` | `filled`, `tonal`, `elevated`, `outlined`, `text`, `danger`, `fab`, `button` |
| Calendar | `widget::calendar` | `calendar`, `date_picker`, `DateRange` |
| Carousel | `widget::carousel` | `carousel`, `Controller`, `PageController` |
| CheckButton | `widget::controls` | `check_button` |
| Checkbox | `widget::toggles` | `checkbox` |
| CheckboxListTile | `widget::list_tile` | `checkbox_list_tile` |
| ChoiceChipBar | `widget::chips` | `choice_chip_bar`, `BarStyle` |
| CodeBlock | `oito2_iced_syntax::code_block` | `code_block` |
| ColorDisk | `widget::controls` | `color_disk` |
| ColorPicker | `widget::color_picker` | `color_picker`, `color_picker_button`, `ColorFormat` |
| ContextMenuRegion | `widget::context_menu` | `context_menu` |
| CopyButton | `widget::copy` | `copy_button`, `copy_field` |
| DateTimeEntry | `widget::entries` | `date_entry`, `time_entry`, `date_time_entry` |
| DiagonalClip | `widget::diagonal_clip` | `diagonal_clip`, `DiagonalCorner` |
| Dialog | `widget::dialog` | `dialog`, `host` |
| DialogTitleBar | `widget::dialog` | `title_bar` |
| Draggable | `widget::draggable` | `draggable`, `Move` |
| EditableLabel | `widget::editable_label` | `editable_label` |
| Expandable | `widget::expandable` | `expandable`, `expandable_with` |
| ExpansionPanel | `widget::expandable` | `expansion_panel`, `set_expanded` |
| FloatingPanel | `widget::floating_panel` | `floating_panel`, `PanelState` |
| FocusBorder | `focus` | `ring`, `focusable` |
| HeaderBar | `window` | `header_bar` |
| HighlightedText | `widget::highlighted_text` | `highlighted_text`, `matches` |
| HoverCard | `widget::hover_card` | `hover_card` |
| IconButton | `widget::button` | `icon_button`, `back_button`, `close_button` |
| InfoBox | `widget::info` | `info_box`, `info_badge`, `InfoType` |
| JsonTreeView | `widget::json_tree_view` | `json_tree_view`, `JsonValue` |
| ListBox | `widget::list_box` | `list_box`, `SelectionChange` |
| ListTile | `widget::list_tile` | `list_tile` |
| Marquee | `widget::marquee` | `marquee`, `Axis` |
| MasterDetailPage | `widget::master_detail` | `master_detail`, `master_tile`, `detail_page` |
| NavigationPage | `widget::navigation_page` | `navigation_page` |
| NavigationRail | `widget::navigation_rail` | `navigation_rail`, `item`, `NavigationRailStyle` |
| NumberEntry | `widget::number_entry` | `number_entry` |
| OptionButton | `widget::controls` | `option_button` |
| PageIndicator | `widget::page_indicator` | `page_indicator`, `PageIndicatorLayout` |
| Pagination | `widget::pagination` | `pagination` |
| PanedView | `widget::paned_view` | `paned_view` |
| PasswordField | `widget::password` | `password_field` |
| PinEntry | `widget::pin_entry` | `pin_entry` |
| PlaceholderIcon | `icon` | `placeholder` |
| Popover | `widget::popover` | `popover` |
| PopupMenuButton | `widget::menu` | `popup_menu_button`, `item`, `checked_item`, `multi_select_item`, `header`, `separator`, `anchor`, `select` |
| ProgressIndicator | `widget::progress` | `linear`, `linear_indeterminate`, `circular`, `circular_indeterminate` |
| Radio | `widget::toggles` | `radio` |
| RadioButton | `widget::controls` | `radio_button` |
| Rating | `widget::rating` | `rating` |
| ScrollArea | `widget::scroll_area` | `scroll_area`, `scrollbar` |
| ScrollViewUndershoot | `widget::scroll_area` | `undershoot` |
| SearchField | `widget::search` | `search_field`, `search_title_field` |
| Section | `widget::containers` | `section` |
| SegmentGroup | `widget::segment_group` | `segment_group`, `segment` |
| SegmentedEntry | `widget::entries` | `segmented_entry`, `segment` |
| SelectableContainer | `widget::selectable` | `selectable_container` |
| Sheet | `widget::sheet` | `sheet`, `content`, `host` |
| Slider | `widget::slider` | `slider`, `range_slider`, `mark` |
| SplitButton | `widget::split_button` | `split_button` |
| Splitter | `widget::splitter` | `splitter`, `pane`, `State` |
| Steps | `widget::steps` | `steps`, `step` |
| Swap | `widget::swap` | `swap`, `SwapEffect` |
| Switch | `widget::toggles` | `switch` |
| SwitchButton | `widget::controls` | `switch_button` |
| TabBar | `widget::tab_bar` | `tab_bar`, `tab`, `TabBarStyle` |
| TableOfContents | `widget::table_of_contents` | `table_of_contents`, `tracker`, `entry`, `scroll_to` |
| TagsField | `widget::tags_field` | `tags_field`, `TagsFieldStyle` |
| TextField | `widget::text_field` | `text_field` |
| Tile | `widget::containers` | `tile`, `tile_list` |
| TimerText | `widget::timer` | `timer_text`, `Timer` |
| TitleBar | `window` | `title_bar`, `controls`, `window_control`, `resize_handles` |
| Toaster | `widget::toast` | `toaster`, `Toasts`, `Toast` |
| ToggleButton | `widget::controls` | `toggle_button` |
| ToggleGroup | `widget::toggle_group` | `toggle_group`, `item`, `icon_item` |
| Tooltip | `widget` | `tooltip` |
| Tour | `widget::tour` | `host`, `Tour`, `Step` |
| TreeView | `widget::tree_view` | `tree_view`, `TreeState` |
| VerticalTabs | `widget::tab_bar` | `vertical_tabs`, `vertical_tab_bar` |
| Watermark | `widget::containers` | `watermark` |

## Extras (`oito2_iced_extras`)

| Página do showcase | Módulo | Itens principais |
|---|---|---|
| ImageCropper | `image_cropper` | `image_cropper`, `preview`, `Crop`, `crop` |
| NavigationMenu | `navigation_menu` | `navigation_menu`, `item`, `MenuState` |
| QrCode | `qr_code` | `qr_code`, `ErrorLevel`, `Modules` |
| SignaturePad | `signature_pad` | `signature_pad`, `Signature` |

## Arquivos (`oito2_iced_files`)

| Página do showcase | Módulo | Itens principais |
|---|---|---|
| DownloadTrigger | `download` | `download_trigger`, `save` |
| FileUpload | `upload`, `drop_zone` | `file_upload`, `Controller`, `drop_zone`, `pick` |

## Sem página própria

| Módulo | Itens | Onde aparece |
|---|---|---|
| `widget::command_palette` | `command_palette` | A página Commands (Utilities) e Ctrl+K |
| `widget` | `focusable`, `min_size`, `styled_text` | Usados pelas outras páginas |
| `overlay` | `anchored`, `place` | Popover, menus, tooltips |
| `widget::helpers` | `ReadingDirection`, `DoubleClick` | — |

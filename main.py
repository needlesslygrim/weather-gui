import weather_cli

import sys
import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")
from gi.repository import Gtk, Adw, Gio, Gdk, Graphene, GLib


class MainWindow(Gtk.ApplicationWindow):
	def __init__(self, *args, **kwargs):
		super().__init__(*args, **kwargs)

		self.set_default_size(600, 250)
		self.set_title("Veður")

		# Main layout containers
		self.box1 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
		self.box2 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
		self.box3 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
		self.box1.set_spacing(10)
		self.box1.set_margin_top(10)
		self.box1.set_margin_bottom(10)
		self.box1.set_margin_start(10)
		self.box1.set_margin_end(10)
		self.box2.set_spacing(10)
		self.box2.set_margin_top(10)
		self.box2.set_margin_bottom(10)
		self.box2.set_margin_start(10)
		self.box2.set_margin_end(10)
		self.box3.set_spacing(10)
		self.box3.set_margin_top(10)
		self.box3.set_margin_bottom(10)
		self.box3.set_margin_start(10)
		self.box3.set_margin_end(10)
		self.set_child(self.box1)  # Horizontal box to window
		self.box1.append(self.box2)  # Put vert box in that box
		self.box1.append(self.box3)  # And another one, empty for now
		self.set_child(self.box1)  # Horizontal box to window

		self.location_entry = Gtk.Entry()
		self.box2.append(self.location_entry)

		self.temp = Gtk.Label()
		self.feels_like = Gtk.Label()
		self.temp_max = Gtk.Label()
		self.temp_min = Gtk.Label()
		self.main = Gtk.Label()
		self.description = Gtk.Label()
		self.box3.append(self.temp)
		self.box3.append(self.feels_like)
		self.box3.append(self.temp_max)
		self.box3.append(self.temp_min)
		self.box3.append(self.main)
		self.box3.append(self.description)

		# Add a button
		self.button = Gtk.Button(label="Get the weather!")
		self.box2.append(
			self.button
		)  # But button in the first of the two vertical boxes
		self.button.connect('clicked', self.get_weather)
		# Add a box containing a switch and label
		self.switch_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL)
		self.switch_box.set_spacing(5)

		self.header = Gtk.HeaderBar()
		self.set_titlebar(self.header)

		# Create a new menu, containing that action
		menu = Gio.Menu.new()

		# Create a popover
		self.popover = Gtk.PopoverMenu()  # Create a new popover menu
		self.popover.set_menu_model(menu)

		# Create a menu button
		self.hamburger = Gtk.MenuButton()
		self.hamburger.set_popover(self.popover)
		self.hamburger.set_icon_name("open-menu-symbolic")  # Give it a nice icon

		# Add menu button to the header bar
		self.header.pack_start(self.hamburger)

		# set app name
		GLib.set_application_name("Weather")

		# Add an about dialog
		action = Gio.SimpleAction.new("about", None)
		action.connect("activate", self.show_about)
		self.add_action(
			action
		)  # Here the action is being added to the window, but you could add it to the
		menu.append("About", "win.about")

		app = self.get_application()
		sm = app.get_style_manager()
		sm.set_color_scheme(Adw.ColorScheme.PREFER_DARK)

	def show_about(self, action, param):
		self.about = Gtk.AboutDialog()
		self.about.set_transient_for(self)
		self.about.set_modal(self)

		self.about.set_authors(["Erick Howard"])
		self.about.set_copyright("Copyright 2022 Erick Thor Howard")
		self.about.set_license_type(Gtk.License.GPL_3_0)
		self.about.set_website("http://github.com/ErickHoward")
		self.about.set_website_label("My Github Profile")
		self.about.set_version("1.0")
		self.about.set_logo_icon_name("org.gnome.weather")

		self.about.show()

	def get_weather(self, action):
		weather = weather_cli.send_request(self.location_entry.get_text(), "e391a6cfbcd81421bbc316f0eb5ab74c")
		self.main.set_text(f"The weather is currently: {weather.weather[0].main}")
		self.description.set_text(f"The weather is more specifically, : {weather.weather[0].description}")
		self.temp.set_text(f"The temperature is: {weather.temp.temp}")
		self.feels_like.set_text(f"The temperature feels like: {weather.temp.feels_like}")
		self.temp_max.set_text(f"The maximum temperature today will be: {weather.temp.temp_max}")
		self.temp_min.set_text(f"The minimum temperature today will be: {weather.temp.temp_min}")


class MyApp(Adw.Application):
	def __init__(self, **kwargs):
		super().__init__(**kwargs)
		self.connect("activate", self.on_activate)

	def on_activate(self, app):
		self.win = MainWindow(application=app)
		self.win.present()


if __name__ == "__main__":
	app = MyApp(application_id="com.github.ErickHoward.weather")
	app.run(sys.argv)

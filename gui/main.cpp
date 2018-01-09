#include "main_window.hpp"
#include "client.hpp"
#include <QtWidgets/QApplication>

int main(int argc, char **argv) {
  ffi::initialize_logging();
  QApplication app(argc, argv);

  MainWindow mainWindow;
  mainWindow.show();

  app.exec();
}

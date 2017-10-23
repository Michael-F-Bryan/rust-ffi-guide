#include "gui/main_window.hpp"

extern "C" {
void hello_world();
}

void MainWindow::onClick() { hello_world(); }

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}
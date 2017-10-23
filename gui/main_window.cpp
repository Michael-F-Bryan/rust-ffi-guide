#include "gui/main_window.hpp"
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Clicked" << std::endl;
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}
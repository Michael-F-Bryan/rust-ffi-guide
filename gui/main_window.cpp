#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;
  Request req("https://google.com/");
  std::cout << "Request created" << std::endl;
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}
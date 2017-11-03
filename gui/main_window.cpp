#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;
  Request req("https://www.rust-lang.org/");
  std::cout << "Sending Request" << std::endl;
  Response res = req.send();
  std::cout << "Received Response" << std::endl;

  std::vector<char> raw_body = res.read_body();
  std::string body(raw_body.begin(), raw_body.end());
  std::cout << "Body:" << std::endl << body << std::endl;
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}
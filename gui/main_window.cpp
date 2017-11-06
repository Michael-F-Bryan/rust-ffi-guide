#include "main_window.hpp"
#include "wrappers.hpp"
#include <QCloseEvent>
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;

  Request req = Request("https://www.rust-lang.org/");
  std::cout << "Sending Request" << std::endl;
  pm.pre_send(req);
  Response res = req.send();
  pm.post_receive(res);
  std::cout << "Received Response" << std::endl;

  std::vector<char> raw_body = res.read_body();
  std::string body(raw_body.begin(), raw_body.end());
  std::cout << "Body:" << std::endl << body << std::endl;
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
  button = new QPushButton("Click Me", this);
  button->show();
  connect(button, SIGNAL(released()), this, SLOT(onClick()));

  pm = PluginManager();
}

void MainWindow::closeEvent(QCloseEvent *event) { pm.unload(); }
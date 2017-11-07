#include "main_window.hpp"
#include "wrappers.hpp"
#include <QtGui/QCloseEvent>
#include <QtWidgets/QFileDialog>
#include <QtWidgets/QFormLayout>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QLabel>
#include <iostream>

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;

  Request req("http://httpbin.org/get");
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
  main_widget = new QWidget;
  QFormLayout *layout = new QFormLayout(main_widget);

  btn_click = new QPushButton("Click Me", main_widget);
  connect(btn_click, SIGNAL(released()), this, SLOT(onClick()));
  btn_click->show();
  layout->addRow(new QLabel(tr("Line 1:")), btn_click);

  btn_plugin = new QPushButton("Load Plugin", main_widget);
  connect(btn_plugin, SIGNAL(released()), this, SLOT(loadPlugin()));
  layout->addRow(new QLabel(tr("Load a Plugin:")), btn_plugin);

  setCentralWidget(main_widget);
}

void MainWindow::closeEvent(QCloseEvent *event) { pm.unload(); }

void MainWindow::loadPlugin() {
  const std::string filename =
      QFileDialog::getOpenFileName(
          this, "Select Plugin", ".",
          "Dynamic Libraries (*.so *.dll);;All Files (*)")
          .toStdString();

  if (!filename.empty()) {
    pm.load_plugin(filename);
  }
}
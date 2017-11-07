#include "main_window.hpp"
#include <QtWidgets/QApplication>

extern "C" {
void init_logging();
}

int main(int argc, char **argv) {
  init_logging();
  QApplication app(argc, argv);

  MainWindow *ui = new MainWindow();
  ui->show();

  app.exec();
  delete ui;
}

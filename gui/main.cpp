#include "main_window.hpp"
#include <QtWidgets/QApplication>

int main(int argc, char **argv) {
  QApplication app(argc, argv);

  MainWindow *ui = new MainWindow();
  ui->show();

  app.exec();
}

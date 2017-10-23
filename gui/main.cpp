#include <QtWidgets/QApplication>
#include <QtWidgets/QTextEdit>

int main(int argc, char **argv) {
  QApplication app(argc, argv);

  QTextEdit editor;
  editor.show();

  app.exec();
}
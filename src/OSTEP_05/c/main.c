#include <fcntl.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
    int pid = fork();
    if (pid < 0) {
        perror("faild to fork.");
        return 0;
    }
    if (pid == 0) {
        int result = close(STDOUT_FILENO);
        if (result != 0) {
            perror("failed to close stdout.");
            return 1;
        }
        int old_mask = umask(0);
        int fd = open("./stdout2.txt", O_CREAT | O_RDWR | O_TRUNC, 0666);
        if (fd < 0) {
            perror("failed to open file.");
            return 2;
        }
        umask(old_mask);
        result = execl("/bin/ls", "ls", "-G", NULL);
        if (result != 0) {
            perror("failed to exec `/bin/ls -G`");
            return 3;
        }
        close(fd);
    }
    if (pid > 0) {
        int status = 0;
        waitpid(pid, &status, 0);
    }
}
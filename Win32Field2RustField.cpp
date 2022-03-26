#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <stdexcept>

int main(int argc, char** argv)
{
	if(argc != 2)
		throw std::runtime_error("ERREUR : Nom de fichier obligatoire.");
	
	std::ostringstream oss;
	
	oss << argv[1];
	
	std::ifstream ifs;
	
	ifs.open(oss.str());
	
	if(!ifs.is_open())
	{
		std::ostringstream ossErr;
		
		ossErr << "ERREUR : Fichier \"" << oss.str() << "\" impossible a ouvrir.";
		
		throw std::runtime_error(ossErr.str().c_str());
	}
	
	oss << "_output.txt";
	
	std::ofstream ofs;
	
	ofs.open(oss.str());
	
	if(!ofs.is_open())
	{
		std::ostringstream ossErr;
		
		ossErr << "ERREUR : Fichier \"" << oss.str() << "\" impossible a ouvrir.";
		
		throw std::runtime_error(ossErr.str().c_str());
	}
	
	std::string ligne;
	
	while(std::getline(ifs, ligne))
	{
		std::stringstream ss(ligne);
		std::string p1;
		std::string p2;
		
		std::getline(ss, p1, ' ');
		std::getline(ss, p2, ' ');
		
		ofs << "pub " << p2 << ": " << p1 << "," << std::endl;
	}
	
	ifs.close();
	ofs.close();
	
	return 0;
}
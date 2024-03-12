use rust_web_ui::*;

pub struct Example4;

pub struct Example4Props {}

impl Default for Example4Props {
	fn default() -> Example4Props {
		Example4Props { }
	}
}

impl Widget<'_> for Example4 {
	type Props = Example4Props;

	fn render(mut ctx: Ctx<'_>, props: Example4Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingY(Size::Exact(48)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-4xl")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Applicant Information"));
							});
							props.child("3", Dynamic).run("p", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("max-w-2xl"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(56))]);
								
								props.child("0", Label).run(|props| props.text("Personal details and application."));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("border-t"), Style::Noop("border-gray-100")]);
							
							props.child("1", Dynamic).run("dl", |mut props| {
								props.styles(&[Style::Noop("divide-y"), Style::Noop("divide-gray-100")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Full name"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("0", Label).run(|props| props.text("Margot Foster"));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Application for"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("0", Label).run(|props| props.text("Backend Developer"));
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Email address"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("0", Label).run(|props| props.text("margotfoster@example.com"));
									});
								});
								props.child("7", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Salary expectation"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("0", Label).run(|props| props.text("$120,000"));
									});
								});
								props.child("9", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("About"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("0", Label).run(|props| props.text("Fugiat ipsum ipsum deserunt culpa aute sint do nostrud anim incididunt cillum culpa consequat. Excepteur qui ipsum aliquip consequat sint. Sit id mollit nulla mollit nostrud in ea officia proident. Irure nostrud pariatur mollit ad adipisicing reprehenderit deserunt qui eu."));
									});
								});
								props.child("11", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("grid")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-4")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(0))])]);
									
									props.child("1", Dynamic).run("dt", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Attachments"));
									});
									props.child("3", Dynamic).run("dd", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))])]);
										
										props.child("1", Dynamic).run("ul", |mut props| {
											props.set_attribute("role", "list");
											props.styles(&[Style::Noop("divide-y"), Style::Noop("divide-gray-100"), Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-gray-200")]);
											
											props.child("1", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingY(Size::Exact(16)), Style::PaddingLeft(Size::Exact(16)), Style::PaddingRight(Size::Exact(20)), Style::Noop("text-sm"), Style::Noop("leading-6")]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Flex, Style::Width(Size::Exact(0)), Style::Noop("flex-1"), Style::ItemsCenter]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("3", Dynamic).run("div", |mut props| {
														props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::Noop("gap-2")]);
														
														props.child("1", Dynamic).run("span", |mut props| {
															props.styles(&[Style::Noop("truncate"), Style::Noop("font-medium")]);
															
															props.child("0", Label).run(|props| props.text("resume_back_end_developer.pdf"));
														});
														props.child("3", Dynamic).run("span", |mut props| {
															props.styles(&[Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
															
															props.child("0", Label).run(|props| props.text("2.4mb"));
														});
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Noop("flex-shrink-0")]);
													
													props.child("1", Dynamic).run("a", |mut props| {
														props.set_attribute("href", "#");
														props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
														
														props.child("0", Label).run(|props| props.text("Download"));
													});
												});
											});
											props.child("3", Dynamic).run("li", |mut props| {
												props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::PaddingY(Size::Exact(16)), Style::PaddingLeft(Size::Exact(16)), Style::PaddingRight(Size::Exact(20)), Style::Noop("text-sm"), Style::Noop("leading-6")]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Flex, Style::Width(Size::Exact(0)), Style::Noop("flex-1"), Style::ItemsCenter]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
													});
													props.child("3", Dynamic).run("div", |mut props| {
														props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Flex, Style::Noop("min-w-0"), Style::Noop("flex-1"), Style::Noop("gap-2")]);
														
														props.child("1", Dynamic).run("span", |mut props| {
															props.styles(&[Style::Noop("truncate"), Style::Noop("font-medium")]);
															
															props.child("0", Label).run(|props| props.text("coverletter_back_end_developer.pdf"));
														});
														props.child("3", Dynamic).run("span", |mut props| {
															props.styles(&[Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
															
															props.child("0", Label).run(|props| props.text("4.5mb"));
														});
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(&[Style::MarginLeft(Size::Exact(16)), Style::Noop("flex-shrink-0")]);
													
													props.child("1", Dynamic).run("a", |mut props| {
														props.set_attribute("href", "#");
														props.styles(&[Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
														
														props.child("0", Label).run(|props| props.text("Download"));
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}

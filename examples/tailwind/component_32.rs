use rust_web_ui::*;

pub struct Example32;

pub struct Example32Props {}

impl Default for Example32Props {
	fn default() -> Example32Props {
		Example32Props { }
	}
}

impl Widget<'_> for Example32 {
	type Props = Example32Props;

	fn render(mut ctx: Ctx<'_>, props: Example32Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("h-[985px]")]), Style::OnScreen(Screen::Large, &[Style::Noop("overflow-y-auto")])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Noop("overflow-hidden"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::Noop("overflow-visible")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(0))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("-z-10"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Icon).run(|mut props| {
						props.style(&[Style::Noop("absolute"), Style::Noop("left-[max(50%,25rem)]"), Style::Noop("top-0"), Style::Noop("h-[64rem]"), Style::Noop("w-[128rem]"), Style::Noop("-translate-x-1/2"), Style::Noop("stroke-gray-200"), Style::NoopGroup("[mask-image", &[Style::Noop("radial-gradient(64rem_64rem_at_top,white,transparent)]")])]);
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("grid"), Style::Noop("max-w-2xl"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("items-start")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-10")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-start-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("row-start-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("mx-auto")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::OnScreen(Screen::Large, &[Style::PaddingRight(Size::Exact(16))])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("max-w-lg")])]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("text-indigo-600")]);
									
									props.child("0", Label).run(|props| props.text("Deploy faster"));
								});
								props.child("3", Dynamic).run("h1", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
									
									props.child("0", Label).run(|props| props.text("A better workflow"));
								});
								props.child("5", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-xl"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(78))]);
									
									props.child("0", Label).run(|props| props.text("Aliquet nec orci mattis amet quisque ullamcorper neque, nibh sem. At arcu, sit dui mi, nibh dui, diam eget aliquam. Quisque id at vitae feugiat egestas."));
								});
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("-ml-12"), Style::Noop("-mt-12"), Style::Padding(Size::Exact(48)), Style::OnScreen(Screen::Large, &[Style::Noop("sticky")]), Style::OnScreen(Screen::Large, &[Style::Noop("top-4")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-start-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("row-span-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("row-start-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("overflow-hidden")])]);
						
						props.child("1", Dynamic).run("img", |mut props| {
							props.styles(&[Style::Noop("w-[48rem]"), Style::Noop("max-w-none"), Style::Noop("rounded-xl"), Style::Color(Color::Fg(100)), Style::Noop("shadow-xl"), Style::Noop("ring-1"), Style::Noop("ring-gray-400/10"), Style::OnScreen(Screen::Small, &[Style::Noop("w-[57rem]")])]);
							props.set_attribute("src", "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png");
							props.set_attribute("alt", "");
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-start-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("row-start-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("mx-auto")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::OnScreen(Screen::Large, &[Style::PaddingRight(Size::Exact(16))])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("max-w-xl"), Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(78)), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-lg")])]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.child("0", Label).run(|props| props.text("Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit. Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id."));
								});
								props.child("3", Dynamic).run("ul", |mut props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::MarginTop(Size::Exact(32)), Style::SpaceY(Size::Exact(32)), Style::TextColor(Color::Fg(67))]);
									
									props.child("1", Dynamic).run("li", |mut props| {
										props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::MarginTop(Size::Exact(4)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.child("0", Dynamic).run("strong", |mut props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Push to deploy."));
											});
											props.child("1", Label).run(|props| props.text("Lorem ipsum, dolor sit amet consectetur adipisicing elit. Maiores impedit perferendis suscipit eaque, iste dolor cupiditate blanditiis ratione."));
										});
									});
									props.child("3", Dynamic).run("li", |mut props| {
										props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::MarginTop(Size::Exact(4)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.child("0", Dynamic).run("strong", |mut props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("SSL certificates."));
											});
											props.child("1", Label).run(|props| props.text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo."));
										});
									});
									props.child("5", Dynamic).run("li", |mut props| {
										props.styles(&[Style::Flex, Style::Noop("gap-x-3")]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::MarginTop(Size::Exact(4)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::Noop("text-indigo-600")]);
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.child("0", Dynamic).run("strong", |mut props| {
												props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Database backups."));
											});
											props.child("1", Label).run(|props| props.text("Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et magna sit morbi lobortis."));
										});
									});
								});
								props.child("5", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(32))]);
									
									props.child("0", Label).run(|props| props.text("Et vitae blandit facilisi magna lacus commodo. Vitae sapien duis odio id et. Id blandit molestie auctor fermentum dignissim. Lacus diam tincidunt ac cursus in vel. Mauris varius vulputate et ultrices hac adipiscing egestas. Iaculis convallis ac tempor et ut. Ac lorem vel integer orci."));
								});
								props.child("7", Dynamic).run("h2", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(64)), Style::Noop("text-2xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("No server? No problem."));
								});
								props.child("9", Dynamic).run("p", |mut props| {
									props.styles(&[Style::MarginTop(Size::Exact(24))]);
									
									props.child("0", Label).run(|props| props.text("Id orci tellus laoreet id ac. Dolor, aenean leo, ac etiam consequat in. Convallis arcu ipsum urna nibh. Pharetra, euismod vitae interdum mauris enim, consequat vulputate nibh. Maecenas pellentesque id sed tellus mauris, ultrices mauris. Tincidunt enim cursus ridiculus mi. Pellentesque nam sed nullam sed diam turpis ipsum eu a sed convallis diam."));
								});
							});
						});
					});
				});
			});
		});
	}
}
